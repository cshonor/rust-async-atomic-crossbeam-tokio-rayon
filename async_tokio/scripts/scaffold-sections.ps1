# async_tokio: scaffold named sections (same rules as atomic)
$ErrorActionPreference = "Stop"
$root = Resolve-Path (Join-Path $PSScriptRoot "..\..\async_tokio")

function Ensure-Section($chapterPath, $slug, $demoMoves) {
  $dir = Join-Path $chapterPath $slug
  $moved = $false
  foreach ($m in $demoMoves) {
    $src = Join-Path $chapterPath $m.From
    if (Test-Path $src) {
      if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Force -Path $dir | Out-Null }
      Move-Item $src (Join-Path $dir $m.To) -Force
      $moved = $true
    }
  }
  if ($moved -and -not (Get-ChildItem $dir -Filter "*.rs" -ErrorAction SilentlyContinue)) {
    Write-Warning "No .rs in $dir after move"
  }
}

# ch01
$ch1 = Join-Path $root "ch01_async_intro"
@(
  @{ Slug = "1.1-what-is-async"; Demos = @(@{ From = "demo.rs"; To = "1.1-what-is-async-stdlib-demo.rs" }) },
  @{ Slug = "1.2-processes"; Demos = @() },
  @{ Slug = "1.3-threads"; Demos = @() },
  @{ Slug = "1.4-async-use-cases"; Demos = @() },
  @{ Slug = "1.5-async-file-io"; Demos = @() },
  @{ Slug = "1.6-http-performance"; Demos = @(@{ From = "ch01_reqwest_join.rs"; To = "1.6-http-performance-reqwest-join-demo.rs" }) }
) | ForEach-Object { Ensure-Section $ch1 $_.Slug $_.Demos }

# ch02
$ch2 = Join-Path $root "ch02_async_rust_core"
@(
  @{ Slug = "2.1-future-trait"; Demos = @(
    @{ From = "ch02_counter_future.rs"; To = "2.1-future-trait-counter-demo.rs" },
    @{ From = "demo.rs"; To = "2.1-future-trait-stdlib-demo.rs" }
  ) },
  @{ Slug = "2.2-pinning"; Demos = @() },
  @{ Slug = "2.3-waker-context"; Demos = @() },
  @{ Slug = "2.4-data-sharing"; Demos = @() }
) | ForEach-Object { Ensure-Section $ch2 $_.Slug $_.Demos }

# ch03-11 primary demo placement
Ensure-Section (Join-Path $root "ch03_custom_task_queue") "3.6-custom-join-macro" @(@{ From = "ch03_join_macro_flume.rs"; To = "3.6-custom-join-macro-flume-demo.rs" }, @{ From = "demo.rs"; To = "3.1-async-queue-stdlib-demo.rs" })
Ensure-Section (Join-Path $root "ch04_network_io_runtime") "4.5-mio-poll" @(@{ From = "ch04_mio_poll_listener.rs"; To = "4.5-mio-poll-listener-demo.rs" }, @{ From = "demo.rs"; To = "4.1-executor-connector-stdlib-demo.rs" })
Ensure-Section (Join-Path $root "ch05_coroutines_generators") "5.3-simple-generator" @(@{ From = "ch05_resume_state_machine.rs"; To = "5.3-simple-generator-state-machine-demo.rs" }, @{ From = "demo.rs"; To = "5.1-coroutines-intro-stdlib-demo.rs" })
Ensure-Section (Join-Path $root "ch06_reactive_async_streams") "6.1-heater-system" @(@{ From = "ch06_callback.rs"; To = "6.1-heater-system-callback-demo.rs" }, @{ From = "ch06_heater_display.rs"; To = "6.1-heater-system-display-demo.rs" }, @{ From = "demo.rs"; To = "6.1-heater-system-stdlib-demo.rs" })
Ensure-Section (Join-Path $root "ch07_tokio_graceful_shutdown") "7.4-graceful-shutdown" @(
  @{ From = "ch07_ctrl_c_shutdown.rs"; To = "7.4-graceful-shutdown-ctrl-c-demo.rs" },
  @{ From = "ch07_local_pool_pinned.rs"; To = "7.2-local-pool-pinned-demo.rs" },
  @{ From = "ch07_runtime_builder_hooks.rs"; To = "7.1-runtime-builder-demo.rs" },
  @{ From = "demo.rs"; To = "7.1-runtime-builder-stdlib-demo.rs" }
)
Ensure-Section (Join-Path $root "ch08_actor_model") "8.1-custom-actor" @(@{ From = "ch08_actor_resp_channel.rs"; To = "8.1-custom-actor-resp-channel-demo.rs" }, @{ From = "demo.rs"; To = "8.1-custom-actor-stdlib-demo.rs" })
Ensure-Section (Join-Path $root "ch09_async_design_patterns") "9.5-retry-pattern" @(@{ From = "ch09_retry_backoff.rs"; To = "9.5-retry-pattern-backoff-demo.rs" }, @{ From = "demo.rs"; To = "9.1-isolated-module-stdlib-demo.rs" })
Ensure-Section (Join-Path $root "ch10_dependency_free_async_server") "10.2-std-runtime" @(
  @{ From = "ch10_std_tcp_nonblocking.rs"; To = "10.2-std-runtime-tcp-nonblocking-demo.rs" },
  @{ From = "ch10_noop_waker_block_on.rs"; To = "10.2-std-runtime-noop-waker-demo.rs" },
  @{ From = "demo.rs"; To = "10.1-basics-stdlib-demo.rs" }
)
Ensure-Section (Join-Path $root "ch11_async_testing_debugging") "11.3-deadlock-probe" @(@{ From = "ch11_timeout_deadlock_probe.rs"; To = "11.3-deadlock-probe-timeout-demo.rs" }, @{ From = "demo.rs"; To = "11.1-sync-testing-stdlib-demo.rs" })

# stub md for ch01 sections if missing
@(
  @{ Ch = $ch1; Files = @("1.1-what-is-async","1.2-processes","1.3-threads","1.4-async-use-cases","1.5-async-file-io","1.6-http-performance") },
  @{ Ch = $ch2; Files = @("2.1-future-trait","2.2-pinning","2.3-waker-context","2.4-data-sharing") }
) | ForEach-Object {
  foreach ($f in $_.Files) {
    $md = Join-Path $_.Ch "$f.md"
    if (-not (Test-Path $md)) {
      $demoDir = Join-Path $_.Ch $f
      $hasDemo = (Test-Path $demoDir) -and @(Get-ChildItem $demoDir -Filter "*.rs" -ErrorAction SilentlyContinue).Count -gt 0
      $demoLine = if ($hasDemo) {
        "> Demo: [$f/](./$f/)"
      } else {
        "> **本节无独立 Demo**。"
      }
      "# $($f -replace '-',' ')`n`n$demoLine`n`n（从 ``本章学习笔记.md`` 拆分精读占位）`n" | Set-Content $md -Encoding UTF8
    }
  }
}

Write-Host "async_tokio scaffold done."
