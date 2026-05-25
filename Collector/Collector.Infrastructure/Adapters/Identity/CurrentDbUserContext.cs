using Collector.Core.Ports.Identity;

namespace Collector.Infrastructure.Adapters.Identity;

public sealed class CurrentDbUserContext : ICurrentDbUserContext
{
    public Guid UserId { get; set; }
    public string Username { get; set; } = null!;
}