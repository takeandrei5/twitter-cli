using Collector.Core.UseCases.Common.CurrentDbUser;
using MediatR;

namespace Collector.Core.UseCases.Posts.Queries.GetPosts;

public sealed record GetPostsQuery(string CreatorUsername, int PageNumber)
    : IRequest<GetPostsQueryResult>, IRequireCurrentDbUser;