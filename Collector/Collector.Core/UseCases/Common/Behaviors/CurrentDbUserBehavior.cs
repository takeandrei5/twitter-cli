using Collector.Core.Errors;
using Collector.Core.Ports;
using Collector.Core.Ports.Identity;
using Collector.Core.Ports.Persistence;
using Collector.Core.UseCases.Common.CurrentDbUser;
using CSharpFunctionalExtensions;
using MediatR;

namespace Collector.Core.UseCases.Common.Behaviors;

public sealed class CurrentDbUserBehavior<TRequest, TResponse>(
    IUnitOfWork unitOfWork,
    ICurrentUser currentUser,
    ICurrentDbUserContext currentDbUserContext)
    : IPipelineBehavior<TRequest, TResponse>
    where TRequest : notnull
{
    public async Task<TResponse> Handle(
        TRequest request,
        RequestHandlerDelegate<TResponse> next,
        CancellationToken cancellationToken)
    {
        if (request is not IRequireCurrentDbUser)
        {
            return await next(cancellationToken);
        }

        var user = await unitOfWork.Users.TryGetWithNoTrackingAsync(
            u => u.Username == currentUser.Username,
            cancellationToken);

        if (user is null)
        {
            throw new InvalidOperationException(
                $"Current user lookup failed for request type {typeof(TRequest).Name}. " +
                "Supported response type for this behavior failure path is UnitResult<ApplicationFailure>.");
        }

        currentDbUserContext.UserId = user.Id;
        currentDbUserContext.Username = user.Username;

        return await next(cancellationToken);
    }
}
