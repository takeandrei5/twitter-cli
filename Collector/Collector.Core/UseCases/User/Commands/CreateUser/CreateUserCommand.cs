using Collector.Core.Errors;
using CSharpFunctionalExtensions;
using MediatR;

namespace Collector.Core.UseCases.User.Commands.CreateUser;

public sealed record CreateUserCommand : IRequest<UnitResult<ApplicationFailure>>;