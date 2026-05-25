using Collector.Core.Ports.Persistence;
using Collector.Core.Ports.Identity;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace Collector.Core.UseCases.Posts.Queries.GetPosts;

public sealed class GetPostsQueryHandler(IUnitOfWork unitOfWork, ICurrentDbUserContext currentDbUserContext)
    : IRequestHandler<GetPostsQuery, GetPostsQueryResult>
{
    private const int PAGE_SIZE = 100;

    public async Task<GetPostsQueryResult> Handle(GetPostsQuery request, CancellationToken cancellationToken)
    {
        var posts = await unitOfWork.UserInterests.Query()
           .AsNoTracking()
           .Include(ui => ui.Creator)
           .ThenInclude(c => c.Posts)
           .Where(ui => ui.UserId == currentDbUserContext.UserId && ui.Creator.Username == request.CreatorUsername)
           .SelectMany(ui => ui.Creator.Posts)
           .Skip((request.PageNumber - 1) * PAGE_SIZE)
           .Take(PAGE_SIZE)
           .OrderByDescending(p => p.CreatedAt)
           .Select(p => new PostDto(p.Id, p.Content, p.ImageUrl, p.CreatedAt))
           .ToListAsync(cancellationToken);

        return new GetPostsQueryResult(posts);
    }
}