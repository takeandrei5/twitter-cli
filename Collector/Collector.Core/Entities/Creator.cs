namespace Collector.Core.Entities;

public sealed class Creator : BaseEntity
{
    public Guid Id { get; set; }
    public required string Username { get; set; }

    public ICollection<UserInterest> Followers { get; set; } = [];
    public ICollection<Post> Posts { get; set; } = [];
}
