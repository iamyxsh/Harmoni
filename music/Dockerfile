# Use an official Go runtime as a parent image
FROM golang:latest

# Set the working directory in the container
WORKDIR /app

# Copy the go.mod and go.sum files to download dependencies
COPY go.mod .
# COPY go.sum .

# Download and install any dependencies
RUN go mod download

# Copy the rest of the application code
COPY . .

# Expose the port your Go gRPC service will listen on
EXPOSE 5001

# Command to run your Go gRPC service with hot reloading using Air
CMD ["air"]
