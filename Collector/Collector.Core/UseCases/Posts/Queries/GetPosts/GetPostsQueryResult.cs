namespace Collector.Core.UseCases.Posts.Queries.GetPosts;

public sealed record GetPostsQueryResult(IEnumerable<PostDto> Posts);

public sealed record PostDto(Guid Id, string Content, string? ImageUrl, DateTime CreatedAt);