using Collector.Core.Errors;
using CSharpFunctionalExtensions;

namespace Collector.Core.Ports.X;

public interface IXDataApiClient
{
    public Task<Result<ActiveUserDto, ApplicationFailure>> GetMyUserAsync(CancellationToken cancellationToken);
}