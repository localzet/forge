#!/usr/bin/env python3
"""Apply and validate actions/runner#4723 on a pinned upstream checkout."""
from pathlib import Path
import sys

root = Path(sys.argv[1])

def replace_once(relative, old, new):
    path = root / relative
    original = path.read_text(encoding="utf-8-sig")
    assert original.count(old) == 1, f"Unexpected upstream context in {relative}: {old[:80]}"
    path.write_text(original.replace(old, new), encoding="utf-8")

schema = "src/Sdk/WorkflowParser/workflow-v1.0.json"
replace_once(schema, '''          "checks": {
            "type": "permission-level-any",
            "description": "Check runs and check suites."
          },
          "contents": {''', '''          "checks": {
            "type": "permission-level-any",
            "description": "Check runs and check suites."
          },
          "code-quality": {
            "type": "permission-level-any",
            "description": "Code quality data."
          },
          "contents": {''')
replace_once(schema, '''          "contents": {
            "type": "permission-level-any",
            "description": "Repository contents, commits, branches, downloads, releases, and merges."
          },
          "deployments": {''', '''          "contents": {
            "type": "permission-level-any",
            "description": "Repository contents, commits, branches, downloads, releases, and merges."
          },
          "copilot-requests": {
            "type": "permission-level-write-or-no-access",
            "description": "GitHub Copilot requests."
          },
          "deployments": {''')

model = "src/Sdk/WorkflowParser/Permissions.cs"
replace_once(model, '            Checks = copy.Checks;\n            Contents = copy.Contents;\n',
    '            Checks = copy.Checks;\n            CodeQuality = copy.CodeQuality;\n            Contents = copy.Contents;\n            CopilotRequests = copy.CopilotRequests;\n')
replace_once(model, '            Checks = permissionLevel;\n            Contents = permissionLevel;\n',
    '            Checks = permissionLevel;\n            CodeQuality = permissionLevel;\n            Contents = permissionLevel;\n            // This scope only supports write or none.\n            CopilotRequests = permissionLevel == PermissionLevel.Write ? PermissionLevel.Write : PermissionLevel.NoAccess;\n')
replace_once(model, '                new KeyValuePair<string, (PermissionLevel, PermissionLevel)>("checks", (left.Checks, right.Checks)),\n                new KeyValuePair<string, (PermissionLevel, PermissionLevel)>("contents", (left.Contents, right.Contents)),\n',
    '                new KeyValuePair<string, (PermissionLevel, PermissionLevel)>("checks", (left.Checks, right.Checks)),\n                new KeyValuePair<string, (PermissionLevel, PermissionLevel)>("code-quality", (left.CodeQuality, right.CodeQuality)),\n                new KeyValuePair<string, (PermissionLevel, PermissionLevel)>("contents", (left.Contents, right.Contents)),\n                new KeyValuePair<string, (PermissionLevel, PermissionLevel)>("copilot-requests", (left.CopilotRequests, right.CopilotRequests)),\n')
replace_once(model, '''        [DataMember(Name = "contents", EmitDefaultValue = false)]
        public PermissionLevel Contents''', '''        [DataMember(Name = "code-quality", EmitDefaultValue = false)]
        public PermissionLevel CodeQuality
        {
            get;
            set;
        }

        [DataMember(Name = "contents", EmitDefaultValue = false)]
        public PermissionLevel Contents''')
replace_once(model, '''        [DataMember(Name = "deployments", EmitDefaultValue = false)]
        public PermissionLevel Deployments''', '''        [DataMember(Name = "copilot-requests", EmitDefaultValue = false)]
        public PermissionLevel CopilotRequests
        {
            get;
            set;
        }

        [DataMember(Name = "deployments", EmitDefaultValue = false)]
        public PermissionLevel Deployments''')

converter = "src/Sdk/WorkflowParser/Conversion/WorkflowTemplateConverter.cs"
replace_once(converter, '''                    case "checks":
                        permissions.Checks = permissionLevel;
                        break;
                    case "contents":''', '''                    case "checks":
                        permissions.Checks = permissionLevel;
                        break;
                    case "code-quality":
                        permissions.CodeQuality = permissionLevel;
                        break;
                    case "contents":''')
replace_once(converter, '''                    case "contents":
                        permissions.Contents = permissionLevel;
                        break;
                    case "deployments":''', '''                    case "contents":
                        permissions.Contents = permissionLevel;
                        break;
                    case "copilot-requests":
                        permissions.CopilotRequests = permissionLevel;
                        break;
                    case "deployments":''')

test_source = r'''using System;
using System.Collections.Generic;
using System.Threading;
using GitHub.Actions.WorkflowParser;
using Newtonsoft.Json;
using Xunit;

namespace GitHub.Runner.Common.Tests.Sdk
{
    public sealed class WorkflowPermissionsL0
    {
        [Theory]
        [Trait("Level", "L0")]
        [Trait("Category", "Sdk")]
        [InlineData("code-quality", "none", PermissionLevel.NoAccess)]
        [InlineData("code-quality", "read", PermissionLevel.Read)]
        [InlineData("code-quality", "write", PermissionLevel.Write)]
        [InlineData("copilot-requests", "none", PermissionLevel.NoAccess)]
        [InlineData("copilot-requests", "write", PermissionLevel.Write)]
        public void ParseNewPermissionsAtWorkflowAndJobLevel(string permission, string level, PermissionLevel expected)
        {
            var yaml = $@"name: Permission test
on: push
permissions:
  {permission}: {level}
jobs:
  test:
    runs-on: ubuntu-latest
    permissions:
      {permission}: {level}
    steps:
      - run: echo ok
";

            var workflow = Parse(yaml);

            Assert.Empty(workflow.Errors);
            Assert.NotNull(workflow.Permissions);
            Assert.Equal(expected, GetPermission(workflow.Permissions, permission));
            var job = Assert.Single(workflow.Jobs);
            Assert.NotNull(job.Permissions);
            Assert.Equal(expected, GetPermission(job.Permissions, permission));
            Assert.Equal(expected, GetPermission(workflow.Permissions.Clone(), permission));
            if (expected != PermissionLevel.NoAccess)
            {
                Assert.Contains($"\"{permission}\"", JsonConvert.SerializeObject(workflow.Permissions));
            }
        }

        [Fact]
        [Trait("Level", "L0")]
        [Trait("Category", "Sdk")]
        public void CopilotRequestsRejectsReadLevel()
        {
            var workflow = Parse(@"on: push
permissions:
  copilot-requests: read
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - run: echo ok
");

            Assert.NotEmpty(workflow.Errors);
        }

        [Theory]
        [Trait("Level", "L0")]
        [Trait("Category", "Sdk")]
        [InlineData(PermissionLevel.Read, PermissionLevel.Read, PermissionLevel.NoAccess)]
        [InlineData(PermissionLevel.Write, PermissionLevel.Write, PermissionLevel.Write)]
        public void ShorthandPermissionsIncludeSupportedScopes(
            PermissionLevel level, PermissionLevel codeQuality, PermissionLevel copilotRequests)
        {
            var permissions = new Permissions(level, includeIdToken: true, includeAttestations: true, includeModels: false);

            Assert.Equal(codeQuality, permissions.CodeQuality);
            Assert.Equal(copilotRequests, permissions.CopilotRequests);
            Assert.Equal(codeQuality, permissions.Clone().CodeQuality);
            Assert.Equal(copilotRequests, permissions.Clone().CopilotRequests);

            var shorthand = level == PermissionLevel.Write ? "write-all" : "read-all";
            var workflow = Parse($@"on: push
permissions: {shorthand}
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - run: echo ok
");

            Assert.Empty(workflow.Errors);
            Assert.Equal(codeQuality, workflow.Permissions.CodeQuality);
            Assert.Equal(copilotRequests, workflow.Permissions.CopilotRequests);
        }

        [Theory]
        [Trait("Level", "L0")]
        [Trait("Category", "Sdk")]
        [InlineData("code-quality", PermissionLevel.Read)]
        [InlineData("copilot-requests", PermissionLevel.Write)]
        public void PermissionBoundaryIncludesNewScopes(string permission, PermissionLevel level)
        {
            var requested = new Permissions();
            if (permission == "code-quality")
            {
                requested.CodeQuality = level;
            }
            else
            {
                requested.CopilotRequests = level;
            }

            Assert.True(requested.ViolatesMaxPermissions(new Permissions(), out var violations));
            Assert.Single(violations);
        }

        private static WorkflowTemplate Parse(string yaml)
        {
            var parser = new WorkflowTemplateParser(
                null,
                null,
                new ParseOptions { SkipReusableWorkflows = true },
                WorkflowFeatures.GetDefaults());

            return parser.LoadWorkflow(
                new InlineFileProvider(yaml),
                "workflow.yml",
                "Write",
                new Dictionary<string, ReferencedWorkflow>(),
                CancellationToken.None);
        }

        private static PermissionLevel GetPermission(Permissions permissions, string permission)
        {
            return permission == "code-quality" ? permissions.CodeQuality : permissions.CopilotRequests;
        }

        private sealed class InlineFileProvider : IFileProvider
        {
            private readonly string _yaml;

            public InlineFileProvider(string yaml)
            {
                _yaml = yaml;
            }

            public string GetFileContent(string path) => _yaml;
            public string ResolvePath(string defaultRoot, string path) => path;
        }
    }
}
'''
target = root / "src/Test/L0/Sdk/WorkflowPermissionsL0.cs"
assert not target.exists(), "Test file already exists in upstream"
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(test_source, encoding="utf-8")

import json
doc = json.loads((root / schema).read_text())
props = doc["definitions"]["permissions-mapping"]["mapping"]["properties"]
assert props["code-quality"]["type"] == "permission-level-any"
assert props["copilot-requests"]["type"] == "permission-level-write-or-no-access"
print("Upstream patch applied and JSON schema validated.")
