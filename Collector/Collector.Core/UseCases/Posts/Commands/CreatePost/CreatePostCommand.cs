using Collector.Core.Errors;
using CSharpFunctionalExtensions;
using MediatR;

namespace Collector.Core.UseCases.Posts.Commands.CreatePost;

public sealed record CreatePostCommand(string CreatorUsername, string Title, string Content, string? ImageUrl)
    : IRequest<UnitResult<ApplicationFailure>>;