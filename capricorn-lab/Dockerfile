
FROM rust:alpine

# Creating a directory to work from
RUN mkdir -p /usr/src/capricorn
WORKDIR /usr/src/capricorn

# Copy our app into that directory
COPY . .

# Build our app
CMD ["cargo", "build"]