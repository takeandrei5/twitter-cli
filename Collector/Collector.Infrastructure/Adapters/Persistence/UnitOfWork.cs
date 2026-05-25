using Collector.Core.Ports.Persistence;
using Microsoft.EntityFrameworkCore.Storage;

namespace Collector.Infrastructure.Adapters.Persistence;

public sealed class UnitOfWork(
    ApplicationDbContext dbContext,
    IUserRepository users,
    ICreatorRepository creators,
    IPostRepository posts,
    IUserInterestRepository userInterests) : IUnitOfWork
{
    private IDbContextTransaction? currentTransaction;

    public IUserRepository Users { get; } = users;
    public ICreatorRepository Creators { get; } = creators;
    public IPostRepository Posts { get; } = posts;
    public IUserInterestRepository UserInterests { get; } = userInterests;
    public bool HasActiveTransaction => currentTransaction is not null;

    public async Task BeginTransactionAsync(CancellationToken cancellationToken = default)
    {
        if (currentTransaction is not null)
        {
            return;
        }

        currentTransaction = await dbContext.Database.BeginTransactionAsync(cancellationToken);
    }

    public async Task CommitTransactionAsync(CancellationToken cancellationToken = default)
    {
        if (currentTransaction is null)
        {
            return;
        }

        try
        {
            await currentTransaction.CommitAsync(cancellationToken);
        }
        finally
        {
            await currentTransaction.DisposeAsync();
            currentTransaction = null;
        }
    }

    public async Task RollbackTransactionAsync(CancellationToken cancellationToken = default)
    {
        if (currentTransaction is null)
        {
            return;
        }

        try
        {
            await currentTransaction.RollbackAsync(cancellationToken);
        }
        finally
        {
            await currentTransaction.DisposeAsync();
            currentTransaction = null;
        }
    }

    public int SaveChanges()
    {
        return dbContext.SaveChanges();
    }

    public Task<int> SaveChangesAsync(CancellationToken cancellationToken = default)
    {
        return dbContext.SaveChangesAsync(cancellationToken);
    }
}