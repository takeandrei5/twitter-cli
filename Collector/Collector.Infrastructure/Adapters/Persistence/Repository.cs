using System.Linq.Expressions;
using Collector.Core.Entities;
using Collector.Core.Ports.Persistence;
using Microsoft.EntityFrameworkCore;

namespace Collector.Infrastructure.Adapters.Persistence;

public class Repository<T>(ApplicationDbContext dbContext) : IRepository<T>
    where T : BaseEntity
{
    protected DbSet<T> DbSet { get; } = dbContext.Set<T>();

    public Task<T?> TryGetAsync(Expression<Func<T, bool>> predicate, CancellationToken cancellationToken = default)
    {
        return DbSet.FirstOrDefaultAsync(predicate, cancellationToken);
    }

    public Task<T?> TryGetWithNoTrackingAsync(Expression<Func<T, bool>> predicate,
        CancellationToken cancellationToken = default)
    {
        return DbSet
           .AsNoTracking()
           .FirstOrDefaultAsync(predicate, cancellationToken);
    }

    public Task<bool> ExistsAsync(Expression<Func<T, bool>> predicate, CancellationToken cancellationToken = default)
    {
        return DbSet.AnyAsync(predicate, cancellationToken);
    }

    public IQueryable<T> Query()
    {
        return DbSet.AsQueryable();
    }

    public void Add(T entity)
    {
        DbSet.Add(entity);
    }

    public void Remove(T entity)
    {
        DbSet.Remove(entity);
    }
}