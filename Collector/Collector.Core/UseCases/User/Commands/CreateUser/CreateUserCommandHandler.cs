using Collector.Core.Errors;
using Collector.Core.Ports;
using Collector.Core.Ports.Persistence;
using CSharpFunctionalExtensions;
using MediatR;

namespace Collector.Core.UseCases.User.Commands.CreateUser;

public sealed class CreateUserCommandHandler(IUnitOfWork unitOfWork, ICurrentUser currentUser)
    : IRequestHandler<CreateUserCommand, UnitResult<ApplicationFailure>>
{
    public async Task<UnitResult<ApplicationFailure>> Handle(CreateUserCommand request,
        CancellationToken cancellationToken)
    {
        var username = currentUser.Username;
        var doesUserExist = await unitOfWork.Users.ExistsAsync(u => u.Username == username, cancellationToken);

        if (doesUserExist)
        {
            return UnitResult.Failure(new ApplicationFailure(ApplicationErrorType.ConflictError,
                "User with this username already exists."));
        }

        unitOfWork.Users.Add(new Entities.User
        {
            Username = username
        });

        await unitOfWork.SaveChangesAsync(cancellationToken);

        return UnitResult.Success<ApplicationFailure>();
    }
}