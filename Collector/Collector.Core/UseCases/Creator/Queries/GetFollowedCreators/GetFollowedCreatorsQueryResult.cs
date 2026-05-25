namespace Collector.Core.UseCases.Creator.Queries.GetFollowedCreators;

public sealed record GetFollowedCreatorsQueryResult(IEnumerable<string> CreatorUsernames);