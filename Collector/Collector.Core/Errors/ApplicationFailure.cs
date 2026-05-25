namespace Collector.Core.Errors;

public sealed record ApplicationFailure(ApplicationErrorType ErrorType, string Message);