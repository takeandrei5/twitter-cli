using System.Text.RegularExpressions;
using FluentValidation;

namespace Collector.Core.UseCases.Posts.Commands.CreatePost;

public sealed class CreatePostCommandValidator : AbstractValidator<CreatePostCommand>
{
    private readonly Regex _urlRegex =
        new(@"^https?:\/\/[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(\/[^\s]*)?\.(?:png|jpg|jpeg|gif|webp|bmp|svg)(\?[^\s]*)?$",
            RegexOptions.IgnoreCase | RegexOptions.Compiled);

    public CreatePostCommandValidator()
    {
        RuleFor(command => command.CreatorUsername)
           .Must(username => !string.IsNullOrWhiteSpace(username))
           .WithMessage("Creator username cannot be empty.")
           .MaximumLength(100);

        RuleFor(command => command.Title)
           .Must(title => !string.IsNullOrWhiteSpace(title))
           .WithMessage("Post title cannot be empty.")
           .MaximumLength(200);

        RuleFor(command => command.Content)
           .Must(content => !string.IsNullOrWhiteSpace(content))
           .WithMessage("Post content cannot be empty.");

        RuleFor(command => command.Content)
           .Matches(_urlRegex)
           .When(imageUrl => imageUrl is not null);
    }
}