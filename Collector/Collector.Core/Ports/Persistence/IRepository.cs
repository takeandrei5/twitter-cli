using System.Linq.Expressions;
using Collector.Core.Entities;

namespace Collector.Core.Ports.Persistence;

public interface IRepository<T>
    where T : BaseEntity
{
    Task<T?> TryGetAsync(Expression<Func<T, bool>> predicate, CancellationToken cancellationToken = default);

    Task<T?> TryGetWithNoTrackingAsync(Expression<Func<T, bool>> predicate,
        CancellationToken cancellationToken = default);

    Task<bool> ExistsAsync(Expression<Func<T, bool>> predicate, CancellationToken cancellationToken = default);

    IQueryable<T> Query();

    void Add(T entity);
    void Remove(T entity);
}