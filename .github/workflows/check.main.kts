#!/usr/bin/env kotlin

@file:DependsOn("io.github.typesafegithub:github-workflows-kt:0.41.0")

import io.github.typesafegithub.workflows.actions.actions.CheckoutV3
import io.github.typesafegithub.workflows.domain.RunnerType
import io.github.typesafegithub.workflows.domain.actions.CustomAction
import io.github.typesafegithub.workflows.domain.triggers.PullRequest
import io.github.typesafegithub.workflows.dsl.expressions.expr
import io.github.typesafegithub.workflows.dsl.workflow
import io.github.typesafegithub.workflows.yaml.toYaml

val workflow = workflow(
    name = "check",
    on = listOf(PullRequest(branches = listOf("main"))),
    sourceFile = __FILE__.toPath()
) {
    job(
        id = "lint-and-test",
        runsOn = RunnerType.UbuntuLatest,
    ) {
        uses(name = "Check out", action = CheckoutV3())
        uses(
            name = "Run cargo deny",
            action = CustomAction(actionOwner = "EmbarkStudios", actionName = "cargo-deny-action", actionVersion = "v1")
        )
        uses(
            name = "Install nix",
            action = CustomAction(
                actionOwner = "cachix",
                actionName = "install-nix-action",
                actionVersion = "v20",
                inputs = mapOf(
                    "github_access_token" to expr { secrets.GITHUB_TOKEN }
                )
            )
        )
        run(
            name = "Run test",
            command = "nix develop --command bash -c \"cargo clippy && cargo test\""
        )
    }
}

println(workflow.toYaml())
