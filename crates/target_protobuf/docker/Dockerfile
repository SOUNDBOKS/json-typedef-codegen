FROM znly/protoc

ARG MAIN
WORKDIR /work
COPY /gen /work/jtd_codegen_e2e/


RUN protoc --c_out=. -I=/work/jtd_codegen_e2e/ /work/jtd_codegen_e2e/index.proto
ENTRYPOINT cat
