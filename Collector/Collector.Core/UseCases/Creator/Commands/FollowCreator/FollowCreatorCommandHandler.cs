using Collector.Core.Entities;
using Collector.Core.Errors;
using Collector.Core.Ports.Identity;
using Collector.Core.Ports.Persistence;
using CSharpFunctionalExtensions;
using MediatR;

namespace Collector.Core.UseCases.Creator.Commands.FollowCreator;

public sealed class FollowCreatorCommandHandler(IUnitOfWork unitOfWork, ICurrentDbUserContext currentDbUserContext)
    : IRequestHandler<FollowCreatorCommand, UnitResult<ApplicationFailure>>
{
    public async Task<UnitResult<ApplicationFailure>> Handle(FollowCreatorCommand request,
        CancellationToken cancellationToken)
    {
        var creatorUsername = request.CreatorUsername.Trim();

        var userId = currentDbUserContext.UserId;

        var creator =
            await unitOfWork.Creators.TryGetWithNoTrackingAsync(c => c.Username == creatorUsername, cancellationToken);
        Guid creatorId;

        if (creator is null)
        {
            creator = new Entities.Creator
            {
                Id = Guid.NewGuid(),
                Username = creatorUsername
            };

            creatorId = creator.Id;

            unitOfWork.Creators.Add(creator);
        }
        else
        {
            var isAlreadyFollowing = await unitOfWork.UserInterests.ExistsAsync(userInterest =>
                    userInterest.UserId == userId && userInterest.CreatorId == creator.Id,
                cancellationToken);

            if (isAlreadyFollowing)
            {
                return UnitResult.Failure(new ApplicationFailure(ApplicationErrorType.ConflictError,
                    "User already follows this creator."));
            }

            creatorId = creator.Id;
        }

        unitOfWork.UserInterests.Add(new UserInterest
        {
            UserId = userId,
            CreatorId = creatorId
        });

        await unitOfWork.SaveChangesAsync(cancellationToken);

        return UnitResult.Success<ApplicationFailure>();
    }
}