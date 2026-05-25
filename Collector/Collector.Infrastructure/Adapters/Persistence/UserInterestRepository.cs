using Collector.Core.Entities;
using Collector.Core.Ports.Persistence;

namespace Collector.Infrastructure.Adapters.Persistence;

public sealed class UserInterestRepository(ApplicationDbContext dbContext)
    : Repository<UserInterest>(dbContext), IUserInterestRepository
{
}