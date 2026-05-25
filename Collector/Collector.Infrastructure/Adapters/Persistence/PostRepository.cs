using Collector.Core.Entities;
using Collector.Core.Ports.Persistence;

namespace Collector.Infrastructure.Adapters.Persistence;

public sealed class PostRepository(ApplicationDbContext dbContext) : Repository<Post>(dbContext), IPostRepository
{
}
