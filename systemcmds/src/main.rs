use std::process::Command;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use anyhow::{Result, Context};
use colored::*;
use chrono::Local;

struct Logger {
    file: std::fs::File,
}

impl Logger {
    fn new() -> Result<Self> {
        let log_dir = "logs";
        std::fs::create_dir_all(log_dir)?;
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let log_file = format!("{}/git_gradle_log_{}.txt", log_dir, timestamp);
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file)?;
            
        Ok(Logger { file })
    }

    fn log(&mut self, message: &str) -> Result<()> {
        // 输出到控制台
        println!("{}", message);
        
        // 写入文件（移除颜色代码）
        let clean_message = strip_ansi_escapes::strip_str(message);
        writeln!(self.file, "{}", clean_message)?;
        self.file.flush()?;
        
        Ok(())
    }
}

fn run_git_command(logger: &mut Logger, cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg(cmd)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute git {}", cmd))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        logger.log(&format!("{}", "Error executing git:".red()))?;
        logger.log(&stderr.red().to_string())?;
    }

    Ok(format!("{}{}",stdout, stderr))
}

fn run_gradle_command(logger: &mut Logger, path: &str, cmd: &str) -> Result<()> {
    logger.log(&format!("\n执行 {} 在 {}", cmd, path))?;
    
    let gradle_cmd = if cfg!(windows) {
        "./gradlew.bat"
    } else {
        "./gradlew"
    };

    let output = Command::new(gradle_cmd)
        .arg(cmd)
        .current_dir(path)
        .output()
        .with_context(|| format!("Failed to execute gradle {} in {}", cmd, path))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        logger.log(&format!("{}", format!("Error executing gradle {} in {}:", cmd, path).red()))?;
        logger.log(&stderr.to_string())?;
        return Err(anyhow::anyhow!("Gradle command failed"));
    }

    logger.log(&stdout.to_string())?;
    Ok(())
}

fn build_projects(logger: &mut Logger) -> Result<()> {
    let projects = vec![
        r"C:\Users\lx\code\ai\ai\IntelligentCenter",
        r"C:\Users\lx\code\ai\ai\AiService",
        r"C:\Users\lx\code\ai\ai\aa\MemoryAtomAbility",
        r"C:\Users\lx\code\ai\ai\aa\SimulateTouchAbilityService",
        r"C:\Users\lx\code\ai\ai\aa\SystemAtomAbility",
        r"C:\Users\lx\code\ai\ai\common\VoiceService",
    ];

    for project in projects {
        logger.log(&format!("\n{}", "=================================================="))?;
        logger.log(&format!("开始构建项目: {}", project))?;

        // 执行 clean
        if let Err(e) = run_gradle_command(logger, project, "clean") {
            logger.log(&format!("{}: {}", "Clean失败".red(), e))?;
            continue;
        }

        // 执行 assRelease
        if let Err(e) = run_gradle_command(logger, project, "assRelease") {
            logger.log(&format!("{}: {}", "构建失败".red(), e))?;
            continue;
        }

        logger.log(&"项目构建完成".green().to_string())?;
    }

    Ok(())
}

fn check_repository_status(status_output: &str) -> (bool, bool, bool) {
    let has_changes = !status_output.contains("nothing to commit, working tree clean");
    let needs_pull = status_output.contains("Your branch is behind");
    let needs_push = status_output.contains("Your branch is ahead");
    
    (has_changes, needs_pull, needs_push)
}

fn status_and_pull(logger: &mut Logger, path: &str) -> Result<()> {
    // 切换到指定目录
    env::set_current_dir(&path)
        .with_context(|| format!("Failed to change directory to {}", path))?;
    
    logger.log(&format!("\n{}", "=================================================="))?;
    logger.log(&format!("Working directory: {}", env::current_dir()?.display().to_string().cyan()))?;
    
    // 执行 git status
    logger.log(&"~~~ Git Status ~~~".yellow().to_string())?;
    let status_output = run_git_command(logger, "status", &[])?;
    
    // 解析状态
    let (has_changes, needs_pull, needs_push) = check_repository_status(&status_output);
    
    // 显示状态摘要
    logger.log(&format!("- 本地是否有未提交的更改: {}", if has_changes { "是".red() } else { "否".green() }))?;
    logger.log(&format!("- 是否需要从远程拉取更新: {}", if needs_pull { "是".yellow() } else { "否".green() }))?;
    logger.log(&format!("- 是否有未推送的提交: {}", if needs_push { "是".bright_purple() } else { "否".green() }))?;
    
    // 如果需要拉取更新，执行 git pull
    if needs_pull {
        logger.log(&format!("\n{}", "~~~ Git Pull ~~~".yellow()))?;
        let pull_output = run_git_command(logger, "pull", &[])?;
        logger.log(&pull_output)?;
    } else {
        logger.log(&"仓库已是最新状态，无需拉取".blue().to_string())?;
    }
    
    Ok(())
}

fn process_all_repositories(logger: &mut Logger) -> Result<()> {
    let repositories = vec![
        r"C:\Users\lx\code\ai\ai\IntelligentCenter",
        r"C:\Users\lx\code\ai\ai\AiService",
        r"C:\Users\lx\code\ai\ai\aa\MemoryAtomAbility",
        r"C:\Users\lx\code\ai\ai\aa\SimulateTouchAbilityService",
        r"C:\Users\lx\code\ai\ai\aa\SystemAtomAbility",
        r"C:\Users\lx\code\ai\ai\common\CommonPlugin",
        r"C:\Users\lx\code\ai\ai\common\ModuleLibs",
        r"C:\Users\lx\code\ai\ai\common\sdk_release",
        r"C:\Users\lx\code\ai\ai\common\VoiceService",
    ];

    for repo in repositories {
        if let Err(e) = status_and_pull(logger, repo) {
            logger.log(&format!("{}: {}", "Error processing repository".red(), e))?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut logger = Logger::new()?;
    
    // 记录开始时间
    let start_time = Local::now();
    logger.log(&format!("开始执行时间: {}", start_time.format("%Y-%m-%d %H:%M:%S")))?;
    
    // 首先执行所有仓库的状态检查和更新
    process_all_repositories(&mut logger)?;
    
    logger.log(&format!("\n{}", "开始执行Gradle构建...".cyan()))?;
    // 然后执行gradle构建
    build_projects(&mut logger)?;
    
    // 记录结束时间
    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    logger.log(&format!("\n执行结束时间: {}", end_time.format("%Y-%m-%d %H:%M:%S")))?;
    logger.log(&format!("总耗时: {} 分 {} 秒", duration.num_minutes(), duration.num_seconds() % 60))?;
    
    logger.log(&format!("\n{}", "所有操作完成！".green()))?;
    Ok(())
}
