# Call like this ./deploy.sh $PROJECT

# Build and publish image to our cloud registry
gcloud builds submit --tag eu.gcr.io/$1/rustservice

# Deploy image to Cloud Run and allow unauthenticated traffic to service
gcloud run deploy rustservice --image eu.gcr.io/$1/rustservice \
  --platform managed --project $1 --region europe-west1 \
  --allow-unauthenticated