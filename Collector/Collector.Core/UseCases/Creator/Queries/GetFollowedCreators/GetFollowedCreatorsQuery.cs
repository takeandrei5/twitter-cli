using Collector.Core.UseCases.Common.CurrentDbUser;
using MediatR;

namespace Collector.Core.UseCases.Creator.Queries.GetFollowedCreators;

public sealed record GetFollowedCreatorsQuery : IRequest<GetFollowedCreatorsQueryResult>, IRequireCurrentDbUser;