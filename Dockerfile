FROM rust:1.59

COPY ./ ./

RUN cargo build --release

RUN chmod +x scripts/run.sh

EXPOSE 8080

CMD ["scripts/run.sh"]
