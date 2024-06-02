// Creating an Enum so that the values for "method" are limited
// Using "string" could allow for anything to be input... don't want that

pub enum Method{
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
