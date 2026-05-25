using System.Text.Json.Serialization;

namespace Collector.Core.Ports.X;

public sealed record ActiveUserDto
{
    [JsonPropertyName("id")]
    public required long Id { get; init; }
    
    [JsonPropertyName("name")]
    public required string Name { get; init; }
    
    [JsonPropertyName("username")]
    public required string Username { get; init; }
}