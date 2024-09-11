mod cli {
    pub mod create_project;
    pub mod input;
}

use cli::input::ProjectDetails;

fn main() {
    let aws_sdks = vec![
        "aws-sdk-s3",              // Amazon Simple Storage Service (S3)
        "aws-sdk-ec2",             // Amazon Elastic Compute Cloud (EC2)
        "aws-sdk-lambda",          // AWS Lambda
        "aws-sdk-dynamodb",        // Amazon DynamoDB
        "aws-sdk-sqs",             // Amazon Simple Queue Service (SQS)
        "aws-sdk-sns",             // Amazon Simple Notification Service (SNS)
        "aws-sdk-iam",             // AWS Identity and Access Management (IAM)
        "aws-sdk-cloudwatch",      // Amazon CloudWatch
        "aws-sdk-rds",             // Amazon Relational Database Service (RDS)
        "aws-sdk-cloudformation",  // AWS CloudFormation
        "aws-sdk-apigateway",      // Amazon API Gateway
        "aws-sdk-sts",             // AWS Security Token Service (STS)
        "aws-sdk-secretsmanager",  // AWS Secrets Manager
        "aws-sdk-ssm",             // AWS Systems Manager (SSM)
        "aws-sdk-elbv2",           // Elastic Load Balancing V2 (ALB/NLB)
        "aws-sdk-ecr",             // Amazon Elastic Container Registry (ECR)
        "aws-sdk-eks",             // Amazon Elastic Kubernetes Service (EKS)
        "aws-sdk-ses",             // Amazon Simple Email Service (SES)
        "aws-sdk-kinesis",         // Amazon Kinesis
        "aws-sdk-sagemaker",       // Amazon SageMaker
        "aws-sdk-athena",          // Amazon Athena
        "aws-sdk-glue",            // AWS Glue
        "aws-sdk-stepfunctions",   // AWS Step Functions
        "aws-sdk-redshift",        // Amazon Redshift
        "aws-sdk-elasticache",     // Amazon ElastiCache
        "aws-sdk-route53",         // Amazon Route 53
        "aws-sdk-firehose",        // Amazon Kinesis Data Firehose
        "aws-sdk-emr",             // Amazon Elastic MapReduce (EMR)
        "aws-sdk-codebuild",       // AWS CodeBuild
        "aws-sdk-codepipeline",    // AWS CodePipeline
        "aws-sdk-cognitoidentity", // Amazon Cognito Identity
    ];
    ProjectDetails::new(aws_sdks).create_project();
}
