using Collector.Core.Entities;
using Collector.Core.Ports.Persistence;
using Microsoft.EntityFrameworkCore;

namespace Collector.Infrastructure.Adapters.Persistence;

public sealed class UserRepository(ApplicationDbContext dbContext) : Repository<User>(dbContext), IUserRepository
{
    public Task<bool> ExistsByUsernameAsync(string username, CancellationToken cancellationToken = default)
    {
        return DbSet.AnyAsync(user => user.Username == username, cancellationToken);
    }
}
