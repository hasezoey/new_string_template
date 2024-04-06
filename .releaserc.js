module.exports = {
	plugins: [
		[
			'@semantic-release/commit-analyzer',
			{
				preset: 'angular',
				releaseRules: [
					{ breaking: true, release: 'major' },
					{ type: 'feat', release: 'minor' },
					{ type: 'fix', release: 'patch' },
					{ type: 'docs', release: false },
					{ type: 'style', release: 'patch' },
					{ type: 'refactor', release: 'patch' },
					{ type: 'perf', release: 'patch' },
					{ type: 'test', release: false },
					{ type: 'chore', release: false },
					{ type: 'deps', release: 'minor' },
					{ type: 'revert', release: 'patch' },
					// dont trigger another release on release commit
					{ type: 'release', release: false }
				],
				parserOpts: {
					noteKeywords: ['BREAKING CHANGE', 'BREAKING CHANGES']
				}
			}
		],
		[
			'@semantic-release/release-notes-generator',
			{
				preset: 'conventionalcommits',
				presetConfig: {
					types: [
						{ type: 'feat', section: 'Features' },
						{ type: 'fix', section: 'Fixes' },
						{ type: 'docs', hidden: true },
						{ type: 'style', section: 'Style' },
						{ type: 'refactor', section: 'Refactor' },
						{ type: 'perf', section: 'Performance' },
						{ type: 'test', hidden: true },
						{ type: 'chore', hidden: true },
						{ type: 'deps', section: 'Dependencies' },
						{ type: 'revert', section: 'Reverts' },
						{ type: 'release', hidden: true }
					]
				}
			}
		],
		[
			"@google/semantic-release-replace-plugin",
			{
				"replacements": [
					{
						"files": [
							"README.md",
							"src/lib.rs"
						],
						"from": "new_string_template = \".*\"",
						// The following convert "1.2.3" to "1.2"
						"to": "new_string_template = \"${nextRelease.version.substring(0, nextRelease.version.lastIndexOf('.'))}\"",
					}
				]
			}
		],
		'@semantic-release/changelog',
		'@semantic-release-cargo/semantic-release-cargo',
		[
			'@semantic-release/git', {
				assets: ['Cargo.toml', 'CHANGELOG.md', 'README.md', 'src/lib.rs'],
				message: 'release: v${nextRelease.version}'
			}
		],
		'@semantic-release/github'
	],
};
