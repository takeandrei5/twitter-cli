using Collector.Core.Errors;
using Collector.Core.Ports.Identity;
using Collector.Core.Ports.Persistence;
using CSharpFunctionalExtensions;
using MediatR;

namespace Collector.Core.UseCases.Creator.Commands.UnfollowCreator;

public sealed class UnfollowCreatorCommandHandler(IUnitOfWork unitOfWork, ICurrentDbUserContext currentDbUserContext)
    : IRequestHandler<UnfollowCreatorCommand, UnitResult<ApplicationFailure>>
{
    public async Task<UnitResult<ApplicationFailure>> Handle(UnfollowCreatorCommand request,
        CancellationToken cancellationToken)
    {
        var creatorUsername = request.CreatorUsername.Trim();
        var userId = currentDbUserContext.UserId;

        var creator =
            await unitOfWork.Creators.TryGetWithNoTrackingAsync(c => c.Username == creatorUsername, cancellationToken);

        if (creator is null)
        {
            return UnitResult.Failure(new ApplicationFailure(ApplicationErrorType.NotFoundError,
                "Creator not found."));
        }

        var userInterest = await unitOfWork.UserInterests.TryGetAsync(userInterest =>
                userInterest.UserId == userId && userInterest.CreatorId == creator.Id,
            cancellationToken);

        if (userInterest is null)
        {
            return UnitResult.Failure(new ApplicationFailure(ApplicationErrorType.ConflictError,
                "User does not follow this creator."));
        }

        unitOfWork.UserInterests.Remove(userInterest);

        await unitOfWork.SaveChangesAsync(cancellationToken);

        return UnitResult.Success<ApplicationFailure>();
    }
}