namespace Collector.Core.Entities;

public sealed class UserInterest : BaseEntity
{
    public Guid UserId { get; set; }
    public Guid CreatorId { get; set; }

    public User User { get; set; } = null!;
    public Creator Creator { get; set; } = null!;
}