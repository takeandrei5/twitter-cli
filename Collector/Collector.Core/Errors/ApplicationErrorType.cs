namespace Collector.Core.Errors;

public enum ApplicationErrorType
{
    ConflictError,
    NotFoundError,
    ValidationError,
    ExternalServiceError,
    DataApiIntegrationError,
    InternalServerError
}