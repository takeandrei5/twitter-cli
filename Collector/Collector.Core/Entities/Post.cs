namespace Collector.Core.Entities;

public sealed class Post : BaseEntity
{
    public Guid Id { get; set; }
    public Guid CreatorId { get; set; }

    public required string Content { get; set; }
    public string? ImageUrl { get; set; }

    public Creator Creator { get; set; } = null!;
}
