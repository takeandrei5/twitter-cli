using Collector.Core.Entities;
using Collector.Core.Ports.Persistence;

namespace Collector.Infrastructure.Adapters.Persistence;

public sealed class CreatorRepository(ApplicationDbContext dbContext) : Repository<Creator>(dbContext), ICreatorRepository
{
}
