#!/usr/bin/env kotlin

@file:DependsOn("io.github.typesafegithub:github-workflows-kt:0.41.0")

import io.github.typesafegithub.workflows.actions.actions.CheckoutV3
import io.github.typesafegithub.workflows.domain.RunnerType
import io.github.typesafegithub.workflows.domain.actions.CustomAction
import io.github.typesafegithub.workflows.domain.triggers.PullRequest
import io.github.typesafegithub.workflows.domain.triggers.Push
import io.github.typesafegithub.workflows.dsl.expressions.Contexts
import io.github.typesafegithub.workflows.dsl.expressions.expr
import io.github.typesafegithub.workflows.dsl.workflow
import io.github.typesafegithub.workflows.yaml.toYaml

val CF_ACCOUNT_ID by Contexts.secrets
val CF_API_KEY by Contexts.secrets

val targetBranches = listOf("main")

val workflow = workflow(
    name = "deploy",
    on = listOf(Push(branches = targetBranches), PullRequest(branches = targetBranches)),
    sourceFile = __FILE__.toPath()
) {
    val lintAndTest = job(
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
            name = "run test",
            command = "nix develop --command bash -c \"cargo test\""
        )
    }

    job(
        id = "deploy",
        runsOn = RunnerType.UbuntuLatest,
        needs = listOf(lintAndTest)
    ) {
        uses(name = "Check out", action = CheckoutV3())
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
            name = "run test",
            command = "nix develop --command bash -c \"CF_ACCOUNT_ID=${expr { CF_ACCOUNT_ID }} && CF_API_KEY=${expr { CF_API_KEY }} && wrangler publish\""
        )
    }
}

println(workflow.toYaml())
