using FluentValidation;

namespace Collector.Core.UseCases.Creator.Commands.FollowCreator;

public sealed class FollowCreatorCommandValidator : AbstractValidator<FollowCreatorCommand>
{
    public FollowCreatorCommandValidator()
    {
        RuleFor(command => command.CreatorUsername)
            .Must(username => !string.IsNullOrWhiteSpace(username))
            .WithMessage("Creator username cannot be empty.")
            .MaximumLength(100);
    }
}
