using Collector.Core.Errors;
using Collector.Core.UseCases.Common.CurrentDbUser;
using CSharpFunctionalExtensions;
using MediatR;

namespace Collector.Core.UseCases.Creator.Commands.FollowCreator;

public sealed record FollowCreatorCommand(string CreatorUsername)
	: IRequest<UnitResult<ApplicationFailure>>, IRequireCurrentDbUser;