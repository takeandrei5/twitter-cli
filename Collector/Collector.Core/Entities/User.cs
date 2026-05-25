namespace Collector.Core.Entities;

public sealed class User : BaseEntity
{
    public Guid Id { get; set; }
    public required string Username { get; set; }

    public ICollection<UserInterest> CreatorsFollowed { get; set; } = [];
}
