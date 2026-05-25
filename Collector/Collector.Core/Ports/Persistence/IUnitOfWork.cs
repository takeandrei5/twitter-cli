namespace Collector.Core.Ports.Persistence;

public interface IUnitOfWork
{
    IUserRepository Users { get; }
    ICreatorRepository Creators { get; }
    IPostRepository Posts { get; }
    IUserInterestRepository UserInterests { get; }

    bool HasActiveTransaction { get; }

    Task BeginTransactionAsync(CancellationToken cancellationToken = default);
    Task CommitTransactionAsync(CancellationToken cancellationToken = default);
    Task RollbackTransactionAsync(CancellationToken cancellationToken = default);

    int SaveChanges();
    Task<int> SaveChangesAsync(CancellationToken cancellationToken = default);
}