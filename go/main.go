package main

import (
	"context"
	"log"
	"net"

	"google.golang.org/grpc"
	pb "grpc-benchmark/pb"
)

const (
	port = ":50051"
)

type server struct {
	pb.UnimplementedExampleServiceServer
}

func (s *server) Call(ctx context.Context, in *pb.Req) (*pb.Resp, error) {
	return &pb.Resp{Payload: "Hello"}, nil
}

func main() {
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterExampleServiceServer(s, &server{})
	log.Printf("server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
