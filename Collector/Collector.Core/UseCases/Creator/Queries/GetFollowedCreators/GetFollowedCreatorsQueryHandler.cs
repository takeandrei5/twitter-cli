using Collector.Core.Ports.Persistence;
using Collector.Core.Ports.Identity;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace Collector.Core.UseCases.Creator.Queries.GetFollowedCreators;

public sealed class GetFollowedCreatorsQueryHandler(
    IUnitOfWork unitOfWork,
    ICurrentDbUserContext currentDbUserContext)
    : IRequestHandler<GetFollowedCreatorsQuery, GetFollowedCreatorsQueryResult>
{
    public async Task<GetFollowedCreatorsQueryResult> Handle(GetFollowedCreatorsQuery request,
        CancellationToken cancellationToken)
    {
        var creatorUserNames = await unitOfWork.UserInterests.Query()
           .AsNoTracking()
           .Where(ui => ui.UserId == currentDbUserContext.UserId)
           .Include(ui => ui.Creator)
           .Select(ui => ui.Creator.Username)
           .ToListAsync(cancellationToken);

        if (creatorUserNames.Count == 0)
        {
            return new GetFollowedCreatorsQueryResult([]);
        }

        return new GetFollowedCreatorsQueryResult(creatorUserNames);
    }
}