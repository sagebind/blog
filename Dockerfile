FROM mcr.microsoft.com/dotnet/sdk:5.0-alpine AS build
WORKDIR /app
COPY . /app/
RUN apk --no-cache add git sassc && \
    dotnet restore && \
    dotnet publish -c Release -o out -r linux-musl-x64

FROM mcr.microsoft.com/dotnet/aspnet:5.0-alpine
RUN apk --no-cache add tzdata
WORKDIR /app
COPY wwwroot /app/wwwroot/
COPY --from=build /app/out /app/
RUN chmod +x /app/blog
CMD ["/app/blog"]
