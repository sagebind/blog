FROM microsoft/dotnet:2.2-sdk-alpine AS build
WORKDIR /app
COPY articles /app/articles/
COPY src /app/src/
COPY *.csproj /app/
RUN dotnet restore && \
    dotnet publish -c Release -o out -r linux-musl-x64

FROM alpine AS css
COPY styles /styles
RUN apk --no-cache add sassc && \
    sassc --style compressed /styles/base.scss /style.min.css

FROM microsoft/dotnet:2.2-runtime-deps-alpine
WORKDIR /app
COPY wwwroot /app/wwwroot/
COPY --from=build /app/out /app/
COPY --from=css /style.min.css /app/wwwroot/assets/css/style.min.css
RUN chmod +x /app/blog
CMD ["/app/blog"]
