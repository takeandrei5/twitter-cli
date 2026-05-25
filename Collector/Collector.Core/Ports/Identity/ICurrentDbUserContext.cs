namespace Collector.Core.Ports.Identity;

public interface ICurrentDbUserContext
{
    Guid UserId { get; set; }
    string Username { get; set; }
}