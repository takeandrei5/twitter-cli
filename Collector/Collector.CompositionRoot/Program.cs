using Collector.Core.Ports;
using Collector.Core.Ports.Identity;
using Collector.Core.Ports.Persistence;
using Collector.Core.UseCases.Common.Behaviors;
using Collector.Core.UseCases.User.Commands.CreateUser;
using FluentValidation;
using Collector.Infrastructure.Adapters;
using Collector.Infrastructure.Adapters.Identity;
using Collector.Infrastructure.Adapters.Persistence;
using Collector.Infrastructure;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

namespace Collector.CompositionRoot;

public static class Program
{
	public static IServiceCollection AddCollectorApiComposition(
		this IServiceCollection services,
		IConfiguration configuration)
	{
		ValidatorOptions.Global.DefaultClassLevelCascadeMode = CascadeMode.Stop;
		ValidatorOptions.Global.DefaultRuleLevelCascadeMode = CascadeMode.Stop;

		var connectionString = configuration.GetConnectionString("DefaultConnection")
			?? throw new InvalidOperationException("Connection string 'DefaultConnection' was not found.");

		services.AddDbContext<ApplicationDbContext>(options => options.UseNpgsql(connectionString));

		services.AddValidatorsFromAssemblyContaining<CreateUserCommand>();
		services.AddMediatR(cfg =>
		{
			cfg.RegisterServicesFromAssembly(typeof(CreateUserCommand).Assembly);
			cfg.AddOpenBehavior(typeof(ValidationBehavior<,>));
			cfg.AddOpenBehavior(typeof(CurrentDbUserBehavior<,>));
		});
		services.AddScoped<ICurrentUser, CurrentUser>();
		services.AddScoped<ICurrentDbUserContext, CurrentDbUserContext>();

		services.AddScoped<IUserRepository, UserRepository>();
		services.AddScoped<ICreatorRepository, CreatorRepository>();
		services.AddScoped<IPostRepository, PostRepository>();
		services.AddScoped<IUserInterestRepository, UserInterestRepository>();
		services.AddScoped<IUnitOfWork, UnitOfWork>();

		return services;
	}
}