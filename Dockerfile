FROM mcr.microsoft.com/dotnet/core/sdk:3.1-alpine AS build
WORKDIR /app
COPY . /app/
RUN apk --no-cache add git && \
    dotnet restore && \
    dotnet publish -c Release -o out -r linux-musl-x64

FROM mcr.microsoft.com/dotnet/core/aspnet:3.1-alpine
RUN apk --no-cache add tzdata
WORKDIR /app
COPY wwwroot /app/wwwroot/
COPY --from=build /app/out /app/
RUN chmod +x /app/blog
CMD ["/app/blog"]
