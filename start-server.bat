@echo off
set SDKWORK_DATABASE_URL=postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev
cd /d E:\sdkwork-space\sdkwork-prompts
target\debug\sdkwork-prompts-api-server.exe
