namespace Collector.Core.Ports;

public interface ICurrentUser
{
    long Id { get; }
    string Name { get; }
    string Username { get; }
}
