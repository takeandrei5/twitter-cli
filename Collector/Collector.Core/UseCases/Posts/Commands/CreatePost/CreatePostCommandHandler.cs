using Collector.Core.Entities;
using Collector.Core.Errors;
using Collector.Core.Ports.Persistence;
using CSharpFunctionalExtensions;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace Collector.Core.UseCases.Posts.Commands.CreatePost;

public sealed class CreatePostCommandHandler(IUnitOfWork unitOfWork)
    : IRequestHandler<CreatePostCommand, UnitResult<ApplicationFailure>>
{
    public async Task<UnitResult<ApplicationFailure>> Handle(CreatePostCommand request,
        CancellationToken cancellationToken)
    {
        var creatorId = await unitOfWork.Creators.Query()
           .AsNoTracking()
           .Where(c => c.Username == request.CreatorUsername)
           .Select(c => c.Id)
           .FirstOrDefaultAsync(cancellationToken);

        if (creatorId == Guid.Empty)
        {
            return UnitResult.Failure(new ApplicationFailure(ApplicationErrorType.NotFoundError,
                "Creator with the specified username does not exist."));
        }

        var post = new Post
        {
            Id = Guid.NewGuid(),
            CreatorId = creatorId,
            Content = request.Content,
            ImageUrl = request.ImageUrl,
            CreatedAt = DateTime.UtcNow
        };

        unitOfWork.Posts.Add(post);
        await unitOfWork.SaveChangesAsync(cancellationToken);

        return UnitResult.Success<ApplicationFailure>();
    }
}