@echo off
set SDKWORK_PROMPTS_DATABASE_URL=postgresql://forum:forum123@localhost:5432/forum
cd /d E:\sdkwork-space\sdkwork-prompts
target\debug\sdkwork-prompts-api-server.exe
