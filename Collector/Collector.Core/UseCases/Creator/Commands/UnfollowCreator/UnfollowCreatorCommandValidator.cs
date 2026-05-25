using FluentValidation;

namespace Collector.Core.UseCases.Creator.Commands.UnfollowCreator;

public sealed class UnfollowCreatorCommandValidator : AbstractValidator<UnfollowCreatorCommand>
{
    public UnfollowCreatorCommandValidator()
    {
        RuleFor(command => command.CreatorUsername)
           .Must(username => !string.IsNullOrWhiteSpace(username))
           .WithMessage("Creator username cannot be empty.")
           .MaximumLength(100);
    }
}