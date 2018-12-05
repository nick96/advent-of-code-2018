pipeline {
	agent {
		docker {
			image "rust:1.30.1"
		}
	}
	
	stages {
		stage("test") {
			steps {
				sh "cargo test"
			}
		}
	}
}
