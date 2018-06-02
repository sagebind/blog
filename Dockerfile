FROM microsoft/dotnet:2.1-sdk AS build
WORKDIR /app
COPY articles /app/articles/
COPY src /app/src/
COPY *.csproj /app/
RUN dotnet restore && \
    dotnet publish -c Release -o out

FROM alpine AS css
COPY styles /styles
RUN apk --no-cache add sassc && \
    sassc --style compressed /styles/base.scss /style.min.css

FROM microsoft/dotnet:2.1-aspnetcore-runtime
WORKDIR /app
COPY wwwroot /app/wwwroot/
COPY --from=build /app/out /app/
COPY --from=css /style.min.css /app/wwwroot/assets/css/style.css
ENTRYPOINT ["dotnet", "blog.dll"]
